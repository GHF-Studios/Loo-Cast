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
    using Attribute;
    using Attribute.Stat;
    using Currency;

    [DisallowMultipleComponent]
    public class Player : ExtendedMonoBehaviour
    {
        public PlayerData Data;
        public PlayerRuntimeData RuntimeData;

        public Dictionary<string, Weapon> Weapons { get; private set; }
        public PlayerHealth Health { get; private set; }
        public PlayerTargeting Targeting { get; private set; }
        public PlayerExperience Experience { get; private set; }
        public PlayerMovement Movement { get; private set; }
        public ParticleSystem ParticleSystem { get; private set; }

        public Stats Stats;
        public Attributes Attributes;
        public Coins Coins;
        public Tokens Tokens;

        private void Awake()
        {
            Weapons = new Dictionary<string, Weapon>();

            MultiplexerWeapon MultiplexerWeapon = GetComponent<MultiplexerWeapon>();
            LaserEmitterWeapon LaserEmitterWeapon = GetComponent<LaserEmitterWeapon>();
            FreezeRayWeapon FreezeRayWeapon = GetComponent<FreezeRayWeapon>();
            ChargedPlasmaLauncherWeapon ChargedPlasmaLauncherWeapon = GetComponent<ChargedPlasmaLauncherWeapon>();

            Weapons.Add("MultiplexerWeapon", MultiplexerWeapon);
            Weapons.Add("LaserEmitterWeapon", LaserEmitterWeapon);
            Weapons.Add("FreezeRayWeapon", FreezeRayWeapon);
            Weapons.Add("ChargedPlasmaLauncherWeapon", ChargedPlasmaLauncherWeapon);

            Health = GetComponent<PlayerHealth>();
            Targeting = GetComponent<PlayerTargeting>();
            Experience = GetComponent<PlayerExperience>();
            Movement = GetComponent<PlayerMovement>();
            ParticleSystem = GetComponentInChildren<ParticleSystem>();
        }

        protected override void OnPauseableUpdate()
        {
            if (Input.GetKeyDown(KeyCode.F1))
            {
                Coins.Balance.Value = Coins.Balance.Value + 1000;
            }

            if (Input.GetKeyDown(KeyCode.F2))
            {
                Coins.Balance.Value = Coins.Balance.Value - 1000;
            }

            if (Input.GetKeyDown(KeyCode.F3))
            {
                Tokens.Balance.Value = Tokens.Balance.Value + 100;
            }

            if (Input.GetKeyDown(KeyCode.F4))
            {
                Tokens.Balance.Value = Tokens.Balance.Value - 100;
            }

            if (Input.GetKeyDown(KeyCode.F5))
            {
                Attributes.Cheat();
                Stats.Cheat();
            }

            if (Input.GetKeyDown(KeyCode.F6))
            {
                Attributes.Uncheat();
                Stats.Uncheat();
            }
        }
    } 
}
