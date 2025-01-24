from django.db import models

class Playlist(models.Model):
    url = models.URLField()
    
    site_url = models.URLField()
    name = models.CharField(max_length=255)

class StreamKey(models.Model):
    url = models.URLField()
    
    method = models.CharField(max_length=255)
    iv = models.BinaryField()
    bytes = models.BinaryField()

class Stream(models.Model):
    playlist = models.ForeignKey(Playlist, on_delete=models.CASCADE)

    url = models.URLField()
    _type = models.CharField(max_length=255),
    version = models.PositiveBigIntegerField(),
    bandwidth = models.PositiveBigIntegerField(),
    codecs = models.CharField(max_length=255),
    resolution = models.CharField(max_length=255),
    target_duration = models.FloatField(),

    key = models.ForeignKey(StreamKey, null=True, on_delete=models.SET_NULL)

class Segment(models.Model):
    stream = models.ForeignKey(Stream, on_delete=models.CASCADE)

    url = models.URLField()
    
    duration = models.FloatField()

class Media(models.Model):
    playlist = models.ForeignKey(Playlist, on_delete=models.CASCADE)

    url = models.URLField()

    _type = models.CharField(max_length=255),
    group_id = models.CharField(max_length=255),   
    name = models.CharField(max_length=255),   
    language = models.CharField(max_length=255),
 
    default = models.BooleanField(),   
    auto_select = models.BooleanField(),   
    forced = models.BooleanField(),
    
class Export(models.Model):
    stream = models.ForeignKey(Stream, on_delete=models.CASCADE)
    media = models.ManyToManyField(Media)

    name = models.CharField(max_length=255)
    path = models.FileField()