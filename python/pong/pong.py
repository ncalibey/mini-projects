import turtle
import os


def create_game_window():
    wn = turtle.Screen()
    wn.title("Pong by @nickcalibey")
    wn.bgcolor("black")
    wn.setup(width=800, height=600)
    wn.tracer(0)

    return wn


def create_paddle(x, y):
    paddle = turtle.Turtle()
    paddle.speed(0)
    paddle.shape("square")
    paddle.color("white")
    paddle.shapesize(stretch_wid=5, stretch_len=1)
    paddle.penup()
    paddle.goto(x, y)

    return paddle


def create_ball():
    ball = turtle.Turtle()
    ball.speed(0)
    ball.shape("square")
    ball.color("white")
    ball.penup()
    ball.goto(0, 0)
    ball.dx = 2
    ball.dy = 2

    return ball


def create_pen():
    pen = turtle.Turtle()
    pen.speed(0)
    pen.shape("square")
    pen.color("white")
    pen.penup()
    pen.hideturtle()
    pen.goto(0, 260)
    pen.write("Player A: 0 Player B: 0", align="center",
              font=("Courier", 24, "normal"))

    return pen


scores = {"score_a": 0, "score_b": 0}
wn = create_game_window()
paddle_a = create_paddle(-350, 0)
paddle_b = create_paddle(350, 0)
ball = create_ball()
pen = create_pen()


def paddle_a_up():
    y = paddle_a.ycor()
    y += 20
    paddle_a.sety(y)


def paddle_a_down():
    y = paddle_a.ycor()
    y -= 20
    paddle_a.sety(y)


def paddle_b_up():
    y = paddle_b.ycor()
    y += 20
    paddle_b.sety(y)


def paddle_b_down():
    y = paddle_b.ycor()
    y -= 20
    paddle_b.sety(y)


def setup_keybindings(wn: turtle._Screen):
    wn.onkeypress(paddle_a_up, "w")
    wn.onkeypress(paddle_a_down, "d")
    wn.onkeypress(paddle_b_up, "Up")
    wn.onkeypress(paddle_b_down, "Down")


def check_y_walls():
    if ball.ycor() > 290:
        ball.sety(290)
        ball.dy *= -1
    elif ball.ycor() < -290:
        ball.sety(-290)
        ball.dy *= -1


def check_x_walls(scores):
    if ball.xcor() > 390:
        scores["score_a"] += 1
        pen.clear()
        pen.write("Player A: {} Player B: {}".format(scores["score_a"], scores["score_b"]),
                  align="center", font=("Courier", 24, "normal"))
        reset_ball_position()
    elif ball.xcor() < -390:
        scores["score_b"] += 1
        pen.clear()
        pen.write("Player A: {} Player B: {}".format(scores["score_a"], scores["score_b"]),
                  align="center", font=("Courier", 24, "normal"))
        reset_ball_position()


def reset_ball_position():
    ball.goto(0, 0)
    ball.dx *= -1


def check_paddle_collision():
    if ball.xcor() < -340 and ball.ycor() < paddle_a.ycor() + 50 and ball.ycor() > paddle_a.ycor() - 50:
        ball.dx *= -1
        os.system("afplay bounce.wav&")
    elif ball.xcor() > 340 and ball.ycor() < paddle_b.ycor() + 50 and ball.ycor() > paddle_b.ycor() - 50:
        ball.dx *= -1
        os.system("afplay bounce.wav&")


wn.listen()
setup_keybindings(wn)

# Ball

##################################################
#### Main Game Loop ##############################
while True:
    wn.update()
    ball.setx(ball.xcor() + ball.dx)
    ball.sety(ball.ycor() + ball.dy)
    check_y_walls()
    check_x_walls(scores)
    check_paddle_collision()
