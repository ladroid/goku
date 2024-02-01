import torch
import torch.nn as nn
import torchvision.transforms as transforms
from torchvision.utils import save_image
from PIL import Image, ImageFilter

# Define the Generator class (must be identical to the one used for training)
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

# Load the trained Generator model
generator = Generator()
generator.load_state_dict(torch.load('/Projects/GameNewAIAlgo/pixel_character_generation/pixel_character_generator_GAN.pth'))
generator.eval()  # Set to evaluation mode

# Function to generate a single image with post-processing
def generate_image(generator, latent_dim=100):
    # Generate random latent vector
    noise = torch.randn(1, latent_dim, 1, 1)

    # Generate an image from the noise vector
    with torch.no_grad():
        generated_image = generator(noise)

    # Post-process the image
    generated_image = (generated_image + 1) / 2  # Rescale images from [-1, 1] to [0, 1]
    generated_image = transforms.ToPILImage()(generated_image.squeeze(0))

    # Apply post-processing
    # Resize using NEAREST to keep pixelated style
    generated_image = generated_image.resize((64, 64), Image.NEAREST)

    # Optional: Apply a sharpening filter to enhance edges
    # You can adjust the parameters or try different filters to see what works best
    generated_image = generated_image.filter(ImageFilter.SHARPEN)

    return generated_image

# Generate and save/show images
# for i in range(5):  # Generate 5 images as an example
#     img = generate_image(generator)
#     img.save(f'generated_character_new{i}.png')  # Save the image
#     img.show()  # Display the image

def launch():
    img = generate_image(generator)
    filename = 'generated_character.png'
    img.save(filename)
    return filename