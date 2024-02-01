import os
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import Dataset, DataLoader
from torchvision import transforms
from PIL import Image

# Function to add random noise to the images
def add_random_noise(img):
    noise = torch.randn(img.size()).mul_(0.1)
    return img.add_(noise)

# Dataset class
class PixelCharactersDataset(Dataset):
    def __init__(self, directory, transform=None):
        self.directory = directory
        self.transform = transform
        self.images = [os.path.join(directory, img) for img in os.listdir(directory) if img.endswith('.png')]

    def __len__(self):
        return len(self.images)

    def __getitem__(self, idx):
        img_name = self.images[idx]
        image = Image.open(img_name).convert('RGB')
        if self.transform:
            image = self.transform(image)
        return image

# Image transformations
transform = transforms.Compose([
    transforms.Resize((32, 32)),
    transforms.RandomHorizontalFlip(),
    transforms.RandomApply([transforms.RandomRotation(10)], p=0.5),
    transforms.RandomApply([transforms.ColorJitter(brightness=0.2, contrast=0.2, saturation=0.2, hue=0.1)], p=0.5),
    transforms.ToTensor(),
    transforms.Lambda(add_random_noise),  # Apply random noise
])

# Load dataset
dataset = PixelCharactersDataset('retro_pixel_characters/images', transform=transform)
dataloader = DataLoader(dataset, batch_size=64, shuffle=True)

# Generator Model
class Generator(nn.Module):
    def __init__(self):
        super(Generator, self).__init__()
        self.model = nn.Sequential(
            # Initial convolution
            nn.ConvTranspose2d(100, 512, 4, 1, 0, bias=False),
            nn.BatchNorm2d(512),
            nn.ReLU(True),

            # Upsampling layers
            nn.Upsample(scale_factor=2, mode='nearest'),
            nn.Conv2d(512, 256, 3, stride=1, padding=1, bias=False),
            nn.BatchNorm2d(256),
            nn.ReLU(True),
            
            nn.Upsample(scale_factor=2, mode='nearest'),
            nn.Conv2d(256, 128, 3, stride=1, padding=1, bias=False),
            nn.BatchNorm2d(128),
            nn.ReLU(True),
            
            nn.Upsample(scale_factor=2, mode='nearest'),
            nn.Conv2d(128, 64, 3, stride=1, padding=1, bias=False),
            nn.BatchNorm2d(64),
            nn.ReLU(True),

            # Final layer to produce an RGB image
            nn.Conv2d(64, 3, 3, stride=1, padding=1, bias=False),
            nn.Tanh()
        )

    def forward(self, x):
        return self.model(x)


# Discriminator Model
class Discriminator(nn.Module):
    def __init__(self):
        super(Discriminator, self).__init__()
        self.model = nn.Sequential(
            nn.Conv2d(3, 128, 4, 2, 1, bias=False),
            nn.LeakyReLU(0.2, inplace=True),
            nn.Conv2d(128, 256, 4, 2, 1, bias=False),
            nn.BatchNorm2d(256),
            nn.LeakyReLU(0.2, inplace=True),
            nn.Conv2d(256, 512, 4, 2, 1, bias=False),
            nn.BatchNorm2d(512),
            nn.LeakyReLU(0.2, inplace=True),
            # The following layer needs to output a single value (1x1x1)
            nn.Conv2d(512, 1, 4, 1, 0, bias=False),
            nn.Flatten(),
            nn.Sigmoid()
        )

    def forward(self, x):
        return self.model(x)


device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")

# Initialize models
generator = Generator()
discriminator = Discriminator()

# Custom weights initialization called on generator and discriminator
def weights_init(m):
    classname = m.__class__.__name__
    if classname.find('Conv') != -1:
        nn.init.normal_(m.weight.data, 0.0, 0.02)
    elif classname.find('BatchNorm') != -1:
        nn.init.normal_(m.weight.data, 1.0, 0.02)
        nn.init.constant_(m.bias.data, 0)

generator.apply(weights_init)
discriminator.apply(weights_init)

# Loss and optimizers
criterion = nn.BCELoss()
optimizerG = optim.Adam(generator.parameters(), lr=0.0002, betas=(0.5, 0.999))
optimizerD = optim.Adam(discriminator.parameters(), lr=0.0002, betas=(0.5, 0.999))

# Training
num_epochs = 200
real_label = 0.9
fake_label = 0.1  # Use soft labels for the GAN training

# Inside the training loop
for epoch in range(num_epochs):
    for i, data in enumerate(dataloader, 0):
        # Train with real images
        discriminator.zero_grad()
        real_images = data.to(device)
        batch_size = real_images.size(0)
        real_labels = torch.full((batch_size,), real_label, dtype=torch.float, device=device)
        real_output = discriminator(real_images).view(-1)  # Get discriminator output for real images
        errD_real = criterion(real_output, real_labels)
        errD_real.backward()
        D_x = real_output.mean().item()  # Use real_output here instead of output

        # Train with fake images
        noise = torch.randn(batch_size, 100, 1, 1, device=device)
        fake_images = generator(noise)
        fake_labels = torch.full((batch_size,), fake_label, dtype=torch.float, device=device)  # Make sure to define fake_labels here
        fake_output = discriminator(fake_images.detach()).view(-1)  # Get discriminator output for fake images
        errD_fake = criterion(fake_output, fake_labels)
        errD_fake.backward()
        D_G_z1 = fake_output.mean().item()

        errD = errD_real + errD_fake
        optimizerD.step()

        # Update Generator: maximize log(D(G(z)))
        generator.zero_grad()
        labels = torch.full((batch_size,), real_label, dtype=torch.float, device=device)  # Labels for generator's cost
        output = discriminator(fake_images).view(-1)
        errG = criterion(output, labels)
        errG.backward()
        D_G_z2 = output.mean().item()
        optimizerG.step()

        if i % 50 == 0:
            print(f'[{epoch}/{num_epochs}][{i}/{len(dataloader)}] Loss_D: {errD.item():.4f} Loss_G: {errG.item():.4f} D(x): {D_x:.4f} D(G(z)): {D_G_z1:.4f}/{D_G_z2:.4f}')

print("Training complete")

# Save models
torch.save(generator.state_dict(), 'pixel_character_generator_GAN.pth')
torch.save(discriminator.state_dict(), 'pixel_character_discriminator.pth')
