using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Currency
{
    using Data;

    public class Tokens : ScriptableObject
    {
        public IntDataReference Balance;
        public IntDataReference ProposedBalanceChange;
    } 
}
