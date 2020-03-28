import 'dart:math';
import 'package:flutter/material.dart';

void main() {
  return runApp(
    MaterialApp(
      home: Scaffold(
        backgroundColor: Colors.red,
        appBar: AppBar(
          title: Text('Dicee'),
          backgroundColor: Colors.red,
        ),
        body: DicePage(),
      ),
    ),
  );
}

class DicePage extends StatefulWidget {
  @override
  _DicePageState createState() => _DicePageState();
}

class _DicePageState extends State<DicePage> {
  int leftDieNumber = 1;
  int rightDieNumber = 1;
  static const int maxNum = 6;
  void changeDie() {
    leftDieNumber = Random().nextInt(maxNum) + 1;
    rightDieNumber = Random().nextInt(maxNum) + 1;
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Row(
        children: <Widget>[
          Expanded(
            child: FlatButton(
              onPressed: () {
                setState(changeDie);
              },
              child: Image.asset('images/dice$leftDieNumber.png'),
            ),
          ),
          Expanded(
            child: FlatButton(
              onPressed: () {
                setState(changeDie);
              },
              child: Image.asset('images/dice$rightDieNumber.png'),
            ),
          ),
        ],
      ),
    );
  }
}
