class Student {
  fullName: string;
  constructor(public firstName: string, public middleInitial: string, public lastName: string) {
    this.fullName = firstName + " " + middleInitial + " " + lastName;
  }
  test() {

  }
}

interface Person {
  firstName: string;
  lastName: string;
}

function test() {

}

function greeter(person: Person) {
  return "Hello, " + person.firstName + " " + person.lastName;
}

let user = new Student("Jane", "M.", "User");
user.test();

document.body.textContent = greeter(user);
