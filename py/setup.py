from setuptools import setup

setup(
   name='nautilus-py',
   version='0.0.1',
   author='Joe Caulfield',
   author_email='jcaulfield135@gmail.com',
   packages=['nautilus', 'nautilus.test'],
   url='https://github.com/nautilus-project/nautilus',
   description='Python client for Nautilus programs on Solana',
   install_requires=[
       "pytest",
       "solana",
   ],
)