import os, sys

# This is let Django knows where to find stuff.
BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

os.environ.setdefault("DJANGO_SETTINGS_MODULE", "officehours.settings")
sys.path.append(BASE_DIR)
os.chdir(BASE_DIR)

# This is allows models to get loaded.
from django.core.wsgi import get_wsgi_application
application = get_wsgi_application()

from logger.models import Session,Log

def start_main_function(): 
    opened_sessions_list = Session.objects.filter(end__isnull=True)
    print("Opened sessions: ")
    for s in opened_sessions_list:
        print(s)

if __name__ == '__main__':
    start_main_function()