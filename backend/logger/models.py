from django.db import models

class Session(models.Model):
    user = models.CharField(max_length=64,default='user')
    start = models.DateTimeField(auto_now=True)
    end = models.DateTimeField(default=None, blank=True, null=True)

class Log(models.Model):
    session = models.ForeignKey(Session, on_delete=models.CASCADE)
    start = models.DateTimeField(auto_now=True)
    end = models.DateTimeField(default=None, blank=True, null=True)