$(function () {
  function getCommentsFor(id) {
    $.ajax({
      url: 'http://localhost:3000/comments',
      data: 'photo_id=' + String(id),
      type: 'GET',
      dataType: 'json'
    }).done(function (json) {
      render.comments(json);
    });
  }

  function getPhotoById(id) {
    id = Number(id);

    return photos.filter(function (photo) { return photo.id === id; })[0];
  }

  function setInputId(id) {
    $("input[name='photo_id']").attr('value', id);
  }

  function resetForm() {
    $('form')[0].reset();
  }

  var templates = {};
  var render = {
    photos: function () {
      $('#slides').html(templates.photos({ photos: photos }));
    },

    photoInformation: function (photo) {
      setInputId(String(photo.id));
      $('section > header').html(templates.photoInformation(photo));
    },

    comments: function (comments) {
      $('div#comments ul').html(templates.comments({ comments: comments }));
    },

    comment: function (comment) {
      $('div#comments ul').append(templates.comment(comment));
    }
  };
  var slideshow = {
    $slideshow: $('#slideshow'),
    duration: 500,
    getCurrentImage: function () {
      return $('#slides figure:visible');
    },
    getNextImage: function($currentImg) {
      var $nextImg;

      if ($currentImg.next().length === 0) {
        $nextImg = this.$slideshow.find('figure').eq(0);
      } else {
        $nextImg = $currentImg.next();
      }

      return $nextImg;
    },
    getPrevImage: function ($currentImg) {
      var $nextImg;

      if ($currentImg.prev().length === 0) {
        $nextImg = this.$slideshow.find('figure').eq(-1);
      } else {
        $nextImg = $currentImg.prev();
      }

      return $nextImg;
    },
    fadeToImage: function($currentImg, $nextImg) {
      $currentImg.fadeOut(this.duration, function() {
        $nextImg.fadeIn(this.duration);
      });
    },
    displayPhotoInformation: function(id) {
      var photo = getPhotoById(id);

      render.photoInformation(photo);
      getCommentsFor(id)
    }
  };
  var actions = {
    increment: function (e) {
      e.preventDefault();

      var $e = $(e.target);
      var id = $e.attr('data-id');

      $.ajax({
        url: $e.attr('href'),
        type: 'POST',
        data: 'photo_id=' + id,
        success: function (count) {
          var photo = getPhotoById(id);

          photo[$e.attr('data-property')] = count.total;
          render.photoInformation(photo);
        }
      });
    },
    addComment: function (e) {
      e.preventDefault();

      var $e = $(e.target);

      $.ajax({
        url: $e.attr('action'),
        type: 'POST',
        data: $('form').serialize(),
        success: function (commentJson) {
          render.comment(commentJson);
          resetForm();
        }
      })
    },
    bindEvents: function () {
      $('section > header').on('click', '.actions a', this.increment.bind(this));
      $('form').on('submit', this.addComment.bind(this));
    },
    init: function () {
      this.bindEvents();
    }
  };
  var photos;

  $('script[type="text/x-handlebars"]').each(function () {
    var $template = $(this);

    templates[$template.attr('id')] = Handlebars.compile($template.html());
  });

  Handlebars.registerPartial('comment', templates.comment);

  $.ajax({
    url: 'http://localhost:3000/photos',
    type: 'GET',
    dataType: 'json',
    success: function (json) {
      photos = json;
      render.photos();
      render.photoInformation(photos[0]);
      getCommentsFor(Number(photos[0].id));
    }
  });

  $('a.next').on('click', function (e) {
    e.preventDefault();

    var $currentImg = slideshow.getCurrentImage();
    var $nextImg = slideshow.getNextImage($currentImg);
    console.log($nextImg);

    slideshow.fadeToImage($currentImg, $nextImg);
    slideshow.displayPhotoInformation(Number($nextImg.attr('data-id')));
  });

  $('a.prev').on('click', function (e) {
    e.preventDefault();

    var $currentImg = slideshow.getCurrentImage();
    var $nextImg = slideshow.getPrevImage($currentImg);
    slideshow.fadeToImage($currentImg, $nextImg);
    slideshow.displayPhotoInformation(Number($nextImg.attr('data-id')));
  });

  actions.init();
});
