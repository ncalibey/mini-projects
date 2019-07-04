var NewAlbumView = Backbone.View.extend({
  template: App.templates.new_album,
  attribute: {
    id: 'album_new'
  },
  events: {
    submit: 'create'
  },

  render () {
    this.$el.html(this.template());
    App.$el.html(this.$el);
  },

  create (e) {
    e.preventDefault();
    var $f = this.$('form');

    $.ajax({
      url: $f.attr('action'),
      type: $f.attr('method'),
      data: $f.serialize(),
      success (json) {
        App.albums.add(json);
        App.indexView();
      }
    });
  },

  initialize () {
    this.render();
  }
});
