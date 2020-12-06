package main

import (
	"fmt"
	"office_hours/model"
	"time"
)

func main() {
	s := model.NewSession(time.Now())
	fmt.Println("Hello World!")
}
