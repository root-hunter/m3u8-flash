from django.db import models

class Playlist(models.Model):
    url = models.URLField()
    site_url = models.URLField()

    name = models.CharField()

class Stream(models.Model):
    pass

class Segment(models.Model):
    pass

class Media(models.Model):
    pass

class Export(models.Model):
    pass