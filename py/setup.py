import pathlib
from setuptools import setup

# The directory containing this file
HERE = pathlib.Path(__file__).parent

# The text of the README file
README = (HERE / "README.md").read_text()

setup(
   name='nautilus_py',
   version='0.0.1',
   author='Joe Caulfield',
   author_email='jcaulfield135@gmail.com',
   packages=['nautilus'],
   url='https://github.com/nautilus-project/nautilus',
   description='Python client for Nautilus programs on Solana',
   install_requires=[
       "pytest",
       "solana",
       "solders",
   ],
)