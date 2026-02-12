class Task:
    def __init__(self, name):
        self.name = name
        self.completed = False
        self.id = 0
        self.tasks = []



    def run(self):
        while True:
            inp = str(input("1: Add task\n2: View task\n3: Leave\nChoose an option: "))
            if inp == "1":
                self.add_task(input("Task name: "))
            elif inp == "2":
                pass
            elif inp == "3":
                break

    def add_task(self,name):
        