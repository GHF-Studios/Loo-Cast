using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Currency
{
    using Data;

    public class Coins : ScriptableObject
    {
        public IntDataReference Balance;
        public IntDataReference ProposedBalanceChange;
    } 
}
