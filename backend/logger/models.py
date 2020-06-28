from django.db import models
from django.utils import timezone

class Session(models.Model):
    user = models.CharField(max_length=64,default='user')
    start = models.DateTimeField(default=timezone.now)
    end = models.DateTimeField(default=None, blank=True, null=True)

    @property
    def total(self):
        return self.end - self.start if (self.end != None) else None
    


class Log(models.Model):
    session = models.ForeignKey(Session, on_delete=models.CASCADE)
    start = models.DateTimeField(default=timezone.now)
    end = models.DateTimeField(default=None, blank=True, null=True)

    @property
    def elapsed(self):
        return self.end - self.start if (self.end != None) else None