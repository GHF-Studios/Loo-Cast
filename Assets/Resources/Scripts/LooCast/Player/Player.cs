using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Player
{
    using Core;
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
    using Game;

    [DisallowMultipleComponent]
    public class Player : ExtendedMonoBehaviour, IItemUpgrader, IGameDataHandler
    {
        #region Structs
        public struct DataContainer
        {
            public Vector3 Position
            {
                get
                {
                    return position;
                }
            }

            [SerializeField] private Vector3 position;

            public DataContainer(Vector3 position)
            {
                this.position = position;
            }
        }
        #endregion

        public PlayerData Data;
        public PlayerRuntimeData RuntimeData;

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

        public RuntimeData GetData()
        {
            DataContainer dataContainer = new DataContainer(transform.position);
            string jsonData = JsonUtility.ToJson(dataContainer);
            RuntimeData gameData = new RuntimeData(jsonData, "Player", "Player", "Prefabs/Player/Player");
            return gameData;
        }

        public void SetData(RuntimeData data)
        {
            DataContainer dataContainer = JsonUtility.FromJson<DataContainer>(data.JsonData);
            transform.position = dataContainer.Position;
        }
    } 
}
