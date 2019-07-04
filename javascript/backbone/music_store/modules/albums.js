var path = require('path');
var fs = require('fs');
var filePath = path.resolve(path.dirname(__dirname), 'data/albums.json');

module.exports = {
  __readFile () {
    return JSON.parse(fs.readFileSync(filePath, 'utf8'))
  },

  getLastID () {
    return this.__readFile().last_id;
  },

  get () {
    return this.__readFile().data;
  },

  set (data) {
    data.id = this.getLastID() + 1;
    fs.writeFileSync(filePath, JSON.stringify({
      last_id: data.id,
      data: data
    }), 'utf8');
  },
};
