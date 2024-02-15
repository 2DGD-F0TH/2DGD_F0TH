Player = {}

function Player:registerShootingObserver(callback)
    -- Function used to register an observer that will be called when the
    -- player shoots a projectile.
end

-- A boss that jumps when the player shoots. Sometimes.
JumpingBoss = {}

function JumpingBoss:new(o)
    o = o or {
        player = Player:new(),
        player_shot = false,
        on_ground = false,
        y_velocity = 0.0
    }
    setmetatable(o, self)
    self.__index = self
    self.player.registerShootingObserver(self.setPlayerShot)
    return o
end

function JumpingBoss:setPlayerShot()
    -- Sets a state that tells the AI that the player shot a bullet
    self.player_shot = true
end

function JumpingBoss:jump()
    -- Sets the boss velocity to -10, making it jump
    if (self.on_ground) then
        self.y_velocity = -10
    end
end

function JumpingBoss:update(dt)
    -- ...
    if (self.player_shot == true) then
        if (math.random(1, 5) == 1) then
            -- Jump only 20% of the times the player shoots
            self:jump()
        end
    end
    -- We reset player_shot to false, if we didn't the boss would jump
    -- a lot more often than 20% of the time
    self.player_shot = false
    -- ...
end
