const { dest, src, series } = require('gulp');
const ts = require('gulp-typescript');
const uglify = require('gulp-uglify');

function htmlTask() {
  return src('src/index.html').pipe(dest('../static'))
}

function tsTask() {
  return src('src/*.ts').pipe(ts({
		noImplicitAny: true,
		outFile: 'index.js'
	})).pipe(uglify()).pipe(dest('../static/'));
}

exports.default = series(htmlTask, tsTask);
