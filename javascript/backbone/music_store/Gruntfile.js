module.exports = function(grunt) {
  grunt.initConfig({
    uglify: {
      my_target: {
        files: {
          'public/javascripts/vendor/all.js': ['public/javascripts/vendor/all.js']
        }
      }
    },
    bower_concat: {
      all: {
        dest: 'public/javascripts/vendor/all.js',
        dependencies: {
          'underscore': 'jquery',
          'backbone': 'underscore'
        }
      }
    },
    handlebars: {
      all: {
        files: {
          'public/javascripts/handlebars_templates.js': ['handlebars/**/*.hbs']
        },
        options: {
          processContent: removeWhitespace,
          processName: extractFileName
        }
      }
    }
  });

  [
    'grunt-bower-concat',
    'grunt-contrib-uglify',
    'grunt-contrib-handlebars'
  ].forEach(function (task) {
    grunt.loadNpmTasks(task);
  });

  grunt.registerTask('default', ['bower_concat', 'uglify']);
};

function removeWhitespace(template) {
  return template.replace(/ {2,}/mg, '').replace(/\r|\n/mg, '');
}

function extractFileName(file) {
  return file.match(/\/(.+)\.hbs$/).pop();
}
