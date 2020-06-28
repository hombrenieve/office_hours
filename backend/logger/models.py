from django.db import models
from django.utils import timezone

def deltaToStr(tdelta):
    hours, rem = divmod(tdelta.seconds, 3600)
    minutes, rem = divmod(rem, 60)
    return "{:02d}:{:02d}".format(hours, minutes)

class Session(models.Model):
    user = models.CharField(max_length=64,default='user')
    start = models.DateTimeField(default=timezone.now)
    end = models.DateTimeField(default=None, blank=True, null=True)

    @property
    def total(self):
        return deltaToStr(self.end - self.start if (self.end != None) else timezone.now() - self.start)

class Log(models.Model):
    session = models.ForeignKey(Session, on_delete=models.CASCADE)
    start = models.DateTimeField(default=timezone.now)
    end = models.DateTimeField(default=None, blank=True, null=True)

    @property
    def elapsed(self):
        return self.end - self.start if (self.end != None) else None