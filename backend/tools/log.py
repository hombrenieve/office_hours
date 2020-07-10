import os, sys

# This is let Django knows where to find stuff.
BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

os.environ.setdefault("DJANGO_SETTINGS_MODULE", "officehours.settings")
sys.path.append(BASE_DIR)
os.chdir(BASE_DIR)

# This is allows models to get loaded.
from django.core.wsgi import get_wsgi_application
application = get_wsgi_application()

import click
from logger.models import Session,Log
from datetime import datetime
from django.utils import timezone

formats='%Y/%m/%d-%H:%M'

def make_aware(dtime):
    return timezone.make_aware(dtime,timezone.get_default_timezone())

@click.group()
def log():
    pass

@log.command()
@click.option('-u', '--user', required=True)
@click.option('--time', type=click.DateTime(formats=[formats,]), default=datetime.now().strftime(formats))
def start(user, time):
    session = Session(user=user, start=make_aware(time))
    session.save()
    log = session.log_set.create(start=make_aware(time))
    log.save()
    click.echo("Session %d by %s started at %s" % (session.id, user, time))

@log.command()
@click.option('-u', '--user', required=True)
@click.option('--time', type=click.DateTime(formats=[formats,]), default=datetime.now().strftime(formats))
def lock(user, time):
    log = Session.objects.filter(end__isnull=True).get(user=user).log_set.get(end__isnull=True)
    log.end = make_aware(time)
    log.save()
    click.echo("Session %d by %s locked at %s" % (log.session.id, user, time))

@log.command()
@click.option('-u', '--user', required=True)
@click.option('--time', type=click.DateTime(formats=[formats,]), default=datetime.now().strftime(formats))
def unlock(user, time):
    log = Session.objects.filter(end__isnull=True).get(user=user).log_set.create(start=make_aware(time))
    log.save()
    click.echo("Session %d by %s unlocked at %s" % (log.session.id, user, time))
    

@log.command()
@click.option('-u', '--user', required=True)
@click.option('--time', type=click.DateTime(formats=[formats,]), default=datetime.now().strftime(formats))
def stop(user, time):
    log = Session.objects.filter(end__isnull=True).get(user=user).log_set.get(end__isnull=True)
    log.end = make_aware(time)
    log.save()
    session = log.session
    session.end = make_aware(time)
    session.save()
    click.echo("Session %d by %s stopped at %s" % (session.id, user, time))

@log.command()
def opened():
    opened_sessions_list = Session.objects.filter(end__isnull=True)
    click.echo("Opened sessions: ")
    for s in opened_sessions_list:
        click.echo(s)


def main(): 
    log()

if __name__ == '__main__':
    main()