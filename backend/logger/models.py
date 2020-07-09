from django.db import models
from django.utils import timezone
from datetime import timedelta

def deltaToStr(tdelta):
    hours, rem = divmod(tdelta.seconds, 3600)
    minutes, rem = divmod(rem, 60)
    return "{:02d}:{:02d}".format(hours, minutes)

def elapsed(holder):
    return holder.end - holder.start if (holder.end != None) else timezone.now() - holder.start

class Session(models.Model):
    user = models.CharField(max_length=64,default='user')
    start = models.DateTimeField(default=timezone.now)
    end = models.DateTimeField(default=None, blank=True, null=True)

    def elapsed(self):
        return elapsed(self)
    
    def working(self):
        delta = timedelta()
        for l in self.log_set.all():
            delta += l.elapsed()
        return delta
    
    def resting(self):
        return self.elapsed() - self.working()
    
    def __str__(self):
        return "Session id(%s) Total(%s) Working(%s) Resting(%s)" % \
            (self.pk, deltaToStr(self.elapsed()), deltaToStr(self.working()), \
                deltaToStr(self.resting()))


class Log(models.Model):
    session = models.ForeignKey(Session, on_delete=models.CASCADE)
    start = models.DateTimeField(default=timezone.now)
    end = models.DateTimeField(default=None, blank=True, null=True)

    def elapsed(self):
        return elapsed(self)