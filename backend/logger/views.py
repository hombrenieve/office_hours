from django.shortcuts import render, get_object_or_404

from .models import Session, Log


def index(request):
    opened_sessions_list = Session.objects.filter(end=None)
    context = {'opened_sessions_list': opened_sessions_list}
    return render(request, 'logger/index.html', context)

def detail(request, session_id):
    session = get_object_or_404(Session, pk=session_id)
    return render(request, 'logger/detail.html', {'session': session})
