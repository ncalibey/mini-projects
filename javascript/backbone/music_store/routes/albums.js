var path = require('path');
var _ = require('underscore');
var Albums = require(path.resolve(path.dirname(__dirname), 'modules/albums'));

module.exports = function (router) {
  router.route('/albums').get(function(req, res) {
    res.json(getAlbums());
  }).post(function (req, res) {
    var album = req.body;
    var albums = Albums.get();

    album.id = Albums.getLastID() + 1;
    albums.push(album);
    Albums.set(albums);
    res.json(album);
  }).put(function (req, res) {
    var albums = Albums.get();
    var currentAlbum = _(albums).findWhere({ id: Number(req.body.id) });

    _.extend(currentAlbum, req.body);
    Albums.set(albums);
    res.json(currentAlbum);
  }).delete(function (req, res) {
    var albums = _(Albums.get()).reject(function (a) {
      return a.id === Number(req.body.id);
    });

    Albums.set(albums);
    res.status(200).end();
  });

  router.get('/albums/new', function (req, res) {
    res.render('new', {
      albums: Albums.get()
    });
  });
};
