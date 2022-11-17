using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Player
{
    using Core;
    using LooCast.Data.Runtime;
    using Data;
    using Data.Runtime;
    using Health;
    using Movement;
    using Particle;
    using Experience;
    using Attribute;
    using Attribute.Stat;
    using Currency;
    using Item;

    [DisallowMultipleComponent]
    [RequireComponent(typeof(PlayerHealth), typeof(PlayerExperience), typeof(PlayerMovement))]
    public class Player : ExtendedMonoBehaviour, IItemUpgrader, IRuntimeDataSerializer, IRuntimeDataDeserializer, IIdentifierProvider, IInstanceIdentifierProvider
    {
        #region Data
        public struct DataContainer
        {
            public PlayerHealth.DataContainer HealthSerializableData => healthSerializableData;
            public Vector3 Position
            {
                get
                {
                    return position;
                }
            }

            [SerializeField] private PlayerHealth.DataContainer healthSerializableData;
            [SerializeField] private Vector3 position;

            public DataContainer(PlayerHealth.DataContainer healthSerializableData, Vector3 position)
            {
                this.healthSerializableData = healthSerializableData;
                this.position = position;
            }
        }

        public DataContainer SerializableData
        {
            get
            {
                return new DataContainer(Health.SerializableData, transform.position);
            }

            set
            {
                Health.SerializableData = value.HealthSerializableData;
                transform.position = value.Position;
            }
        }
        public RuntimeData SerializableRuntimeData
        {
            get
            {
                return new RuntimeData(JsonUtility.ToJson(SerializableData), InstanceIdentifier);
            }

            set
            {
                SerializableData = JsonUtility.FromJson<DataContainer>(value.JsonSerializedData);
            }
        }
        public Identifier Identifier
        {
            get
            {
                return new Identifier(typeof(Player));
            }
        }
        public InstanceIdentifier InstanceIdentifier
        {
            get
            {
                return new InstanceIdentifier(InstanceID, typeof(Player), "Prefabs/Player/Player");
            }
        }

        public PlayerData Data;
        public PlayerRuntimeData RuntimeData;
        #endregion

        public PlayerHealth Health { get; private set; }
        public PlayerExperience Experience { get; private set; }
        public PlayerMovement Movement { get; private set; }
        public ParticleSystem ParticleSystem { get; private set; }
        
        public UpgradeSet UpgradeSet { get; private set; }

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
