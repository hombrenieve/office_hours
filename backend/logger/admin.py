from django.contrib import admin

from .models import Session
from .models import Log

admin.site.register(Session)
admin.site.register(Log)
