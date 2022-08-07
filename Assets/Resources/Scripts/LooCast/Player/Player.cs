using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Player
{
    using Core;
    using Data;
    using Data.Runtime;
    using Health;
    using Targeting;
    using Movement;
    using Weapon;
    using Particle;
    using Experience;
    using Attribute.Stat;
    using Currency;

    [RequireComponent(typeof(PlayerHealth), typeof(Targeting), typeof(PlayerMovement)), DisallowMultipleComponent]
    public class Player : ExtendedMonoBehaviour
    {
        public PlayerData Data;
        public PlayerRuntimeData RuntimeData;

        public Dictionary<string, Weapon> Weapons { get; private set; }
        public PlayerHealth Health { get; private set; }
        public Targeting Targeting { get; private set; }
        public PlayerExperience Experience { get; private set; }
        public PlayerMovement Movement { get; private set; }
        public ParticleSystem ParticleSystem { get; private set; }

        public Stats Stats;
        public Tokens Tokens;
        public Coins Coins;

        private void Awake()
        {
            Weapons = new Dictionary<string, Weapon>();

            MultiplexerWeapon multiplexerWeapon = gameObject.AddComponent<MultiplexerWeapon>();
            LaserEmitterWeapon laserEmitterWeapon = gameObject.AddComponent<LaserEmitterWeapon>();
            FreezeRayWeapon freezeRayWeapon = gameObject.AddComponent<FreezeRayWeapon>();
            ChargedPlasmaLauncherWeapon chargedPlasmaLauncherWeapon = gameObject.AddComponent<ChargedPlasmaLauncherWeapon>();

            Weapons.Add("MultiplexerWeapon", multiplexerWeapon);
            Weapons.Add("LaserEmitterWeapon", laserEmitterWeapon);
            Weapons.Add("FreezeRayWeapon", freezeRayWeapon);
            Weapons.Add("ChargedPlasmaLauncherWeapon", chargedPlasmaLauncherWeapon);

            Health = GetComponent<PlayerHealth>();
            Targeting = GetComponent<Targeting>();
            Experience = GetComponent<PlayerExperience>();
            Movement = GetComponent<PlayerMovement>();
            Movement.OnMovementDisabled.AddListener(ParticleSystem.PauseParticleSpawning);
            Movement.OnMovementEnabled.AddListener(ParticleSystem.ResumeParticleSpawning);
            Movement.OnStartAccelerating.AddListener(ParticleSystem.ResumeParticleSpawning);
            Movement.OnStopAccelerating.AddListener(ParticleSystem.PauseParticleSpawning);
            ParticleSystem = GetComponentInChildren<ParticleSystem>();
        }

        protected override void OnPauseableUpdate()
        {
            if (Input.GetKeyDown(KeyCode.F1))
            {
                Tokens.Balance.Value = Tokens.Balance.Value + 100;
            }

            if (Input.GetKeyDown(KeyCode.F2))
            {
                Tokens.Balance.Value = Tokens.Balance.Value - 100;
            }

            if (Input.GetKeyDown(KeyCode.F3))
            {
                Coins.Balance.Value = Coins.Balance.Value + 1000;
            }

            if (Input.GetKeyDown(KeyCode.F4))
            {
                Coins.Balance.Value = Coins.Balance.Value - 1000;
            }

            if (Input.GetKeyDown(KeyCode.F5))
            {
                Stats.Cheat();
            }

            if (Input.GetKeyDown(KeyCode.F6))
            {
                Stats.Uncheat();
            }
        }
    } 
}
