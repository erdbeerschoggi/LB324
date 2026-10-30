import datetime
from dataclasses import dataclass

items = []

@dataclass
class Item:
    text: str
    date: datetime.date
    isCompleted: bool = False

def add(text, date_string):
    # Text ersetzen und Datum umwandeln
    text = text.replace('b', 'bbb').replace('B', 'Bbb')
    parsed_date = datetime.datetime.strptime(date_string, '%Y-%m-%d').date()
    
    # Item hinzufügen
    items.append(Item(text, parsed_date))
    
    # Liste nach Datum sortieren (für test_sort)
    items.sort(key=lambda x: x.date)

def get_all():
    return items

def get(index):
    return items[index]

def update(index):
    items[index].isCompleted = not items[index].isCompleted