+++
title = "API Python"
classes = ["no_title"]
+++

# Côté Python

```python
from toboggan_py import Toboggan

tbg = Toboggan("localhost", 8080)

print(f"talk:  {tbg.talk}")
print(f"state: {tbg.state}")

tbg.next()           # avance d'une slide
tbg.previous()       # recule
tbg.goto(5)          # slide précise
tbg.blink()          # effet visuel
```

<!-- pause -->

→ **API Pythonique**, **moteur Rust**, **WebSocket temps réel**.
→ Cette présentation est pilotée par ce code.

<!-- notes -->

- API qui ressemble à n'importe quelle lib Python idiomatique
- Aucune fuite de Rust vers l'utilisateur Python
- Sous le capot : WebSocket avec le serveur, sync entre tous les clients
- La présentation actuelle est pilotée via cette API !
- Si live demo : montrer `tbg.next()` et la slide qui change
