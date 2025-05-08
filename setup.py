from setuptools import setup, find_namespace_packages

# read from src/meta.hpp
def read_version():
    with open('src/meta.hpp', 'r') as f:
        for line in f:
            if line.startswith('#define PLUGIN_VERSION'):
                return line.split()[2].strip('"')
    return '0.0.0'

def read_requirements():
    with open('requirements.txt', 'r', encoding='utf-16le') as f:
        return [
            ''.join((
                char for char in line.strip() if char.isascii()
            ))
            for line in f.readlines()
            if not line in (
                'setuptools'
            )
        ]

setup(
    name='vcmp-python-plugin',
    description='A Python plugin for VCMP',
    url='https://github.com/tianxiu2b2t/vcmp-python-plugin',
    author='tianxiu2b2t',
    author_email='administrator@ttb-network.top',
    version=read_version(),
    license='MIT',
    packages=[
        package for package in find_namespace_packages()
        if package.startswith("__vcmp") or package.startswith("vcmp")
    ],
    package_data={
        "": ["libraries/*.so", "libraries/*.dll"],
    },
    install_requires=read_requirements()
)