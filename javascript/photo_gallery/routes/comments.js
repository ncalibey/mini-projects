var express = require('express');
var router = express.Router();
var fs = require("fs");
var path = require("path");
var _ = require("underscore");
var md5 = require("MD5");

var comments = JSON.parse(fs.readFileSync(path.resolve(path.dirname(__dirname), "data/comments.json"), "utf8"));

function convertToGravatar(email) {
  return "http://www.gravatar.com/avatar/" + md5(email.toLowerCase().trim());
}

function getFormattedDate() {
  var date = new Date(),
      year = (date.getFullYear() + "").split("").slice(2).join("");

  return (date.getMonth() + 1) + "/" + date.getDate() + "/" + year;
}

router.get("/comments", function(req, res) {
  var photo_comments = _.where(comments, { photo_id: +req.query.photo_id });
  res.json(photo_comments);
});

router.post("/comments/new", function(req, res) {
  var comment = {};

  comment.body = req.body.body;
  comment.name = req.body.name;
  comment.date = getFormattedDate();
  comment.photo_id = +req.body.photo_id;
  comment.gravatar = convertToGravatar(req.body.email);
  comments.push(comment);

  res.json(comment);
});

module.exports = router;
