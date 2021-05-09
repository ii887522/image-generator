python -m pip install virtualenv
virtualenv env
env\Scripts\activate && python -m pip install cpplint && env\Scripts\deactivate && docker run --rm --name image-generator_installer -w C:\image-generator -v %CD%:C:\image-generator image-generator-custom-node ncu.cmd -u && npm.cmd install
