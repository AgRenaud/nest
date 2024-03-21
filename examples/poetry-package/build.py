import os
import setuptools

def build(setup_kwargs):
    try:
        setuptools.setup(
            **setup_kwargs, 
            script_args = ['bdist_wheel'],
            options = { 
                'bdist_wheel': { 'plat_name': os.getenv('PP_PYTHON_TARGET', 'any') },
                'egg_info': { 'egg_base': './build/' }
            }
        )
    except:
        print("Failed to create targeted wheel")
