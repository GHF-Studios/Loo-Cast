using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Station
{
    using Data;
    using Targeting;
    using Weapon;
    using Health;

    [RequireComponent(typeof(PlayerStationHealth), typeof(Targeting), typeof(MultiplexerWeapon))]
    public sealed class PlayerStation : Station
    {
        public PlayerStationData Data;
        public PlayerStationHealth Health { get; private set; }
        public Targeting Targeting { get; private set; }
        public MultiplexerWeapon DefensiveWeapon { get; private set; }

        private void Start()
        {
            Initialize(Data);

            Health = GetComponent<PlayerStationHealth>();
            Targeting = GetComponent<Targeting>();
            DefensiveWeapon = GetComponent<MultiplexerWeapon>();
        }
    } 
}
