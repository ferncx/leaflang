fnc lineout(message) [void] {

}

fnc hello(message, number) [bool] {
    lineout(message)
    lineout(number)
    ret(true)
}

fnc main() [void] {
    hello("hi!", 2)
}