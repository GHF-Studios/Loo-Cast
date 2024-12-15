using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Player
{
    using LooCast.System;
    using LooCast.System.Identifiers;
    using LooCast.Core;
    using LooCast.Player.Data;
    using LooCast.Player.Data.Runtime;
    using LooCast.Data.Runtime;
    using LooCast.Health;
    using LooCast.Movement;
    using LooCast.Particle;
    using LooCast.Experience;
    using LooCast.Attribute;
    using LooCast.Attribute.Stat;
    using LooCast.Currency;
    using LooCast.Item;

    [DisallowMultipleComponent]
    [RequireComponent(typeof(PlayerHealth), typeof(PlayerExperience), typeof(PlayerMovement))]
    public class Player : ExtendedMonoBehaviour, IItemUpgrader
    {
        public PlayerHealth Health { get; private set; }
        public PlayerExperience Experience { get; private set; }
        public PlayerMovement Movement { get; private set; }
        public ParticleSystem ParticleSystem { get; private set; }
        public UpgradeSet UpgradeSet { get; private set; }

        public PlayerData Data;
        public PlayerRuntimeData RuntimeData;
        public Stats Stats;
        public Attributes Attributes;
        public Coins Coins;
        public Tokens Tokens;

        private void Awake()
        {
            Health = GetComponent<PlayerHealth>();
            Experience = GetComponent<PlayerExperience>();
            Movement = GetComponent<PlayerMovement>();
            ParticleSystem = GetComponentInChildren<ParticleSystem>();

            UpgradeSet = new UpgradeSet(Stats);
        }

        protected override void PauseableUpdate()
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
