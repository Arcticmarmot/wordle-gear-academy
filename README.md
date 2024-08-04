WORDLE-GAME

1. 用户与GameSession程序交互可以发送以下三种类型操作
(1). StartGame 开始游戏
(2). CheckWord 猜测单词
(3). CheckGameStatus 根据用户ID检查游戏状态

2. GameSession程序会返回以下四种类型事件
(1). GameStarted 游戏开始结果
(2). WordChecked 猜测单词结果
(3). GameStatus 游戏状态结果
(4). GameError 发生错误结果