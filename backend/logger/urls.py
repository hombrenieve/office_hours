from django.urls import path

from . import views

app_name = 'logger'
urlpatterns = [
    path('', views.index, name='index'),
    path('history', views.history, name='history'),
    path('<int:session_id>/', views.detail, name='detail'),
]