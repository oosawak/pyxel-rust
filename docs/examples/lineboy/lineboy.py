import pyxel

# Simple Lineboy implementation for WASM
class Game:
    def __init__(self):
        pyxel.init(160, 120)
        self.player_x = 80
        self.player_y = 60
        self.player_vx = 0
        self.player_vy = 0
        self.gravity = 0.2
        self.on_ground = False
        self.score = 0
        
    def update(self):
        # Input
        if pyxel.btn(pyxel.KEY_LEFT) or pyxel.btn(pyxel.KEY_A):
            self.player_vx = -2
        elif pyxel.btn(pyxel.KEY_RIGHT) or pyxel.btn(pyxel.KEY_D):
            self.player_vx = 2
        else:
            self.player_vx = 0
            
        if pyxel.btnp(pyxel.KEY_Z) and self.on_ground:
            self.player_vy = -5
            self.on_ground = False
        
        # Physics
        self.player_vy += self.gravity
        self.player_x += self.player_vx
        self.player_y += self.player_vy
        
        # Ground collision
        if self.player_y >= 100:
            self.player_y = 100
            self.player_vy = 0
            self.on_ground = True
        else:
            self.on_ground = False
            
        # Boundary
        if self.player_x < 4:
            self.player_x = 4
        if self.player_x > 156:
            self.player_x = 156
            
    def draw(self):
        pyxel.cls(0)
        
        # Ground
        pyxel.rect(0, 105, 160, 15, 5)
        
        # Player (neon style)
        pyxel.rect(int(self.player_x)-4, int(self.player_y)-4, 8, 8, 11)
        pyxel.rect(int(self.player_x)-3, int(self.player_y)-3, 6, 6, 15)
        
        # UI
        pyxel.text(5, 5, "LINEBOY", 11)
        pyxel.text(5, 15, f"Y:{self.player_y:.0f}", 11)

def main():
    game = Game()
    pyxel.run(game.update, game.draw)

if __name__ == '__main__':
    main()
