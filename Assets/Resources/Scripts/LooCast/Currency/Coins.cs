using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Currency
{
    using Variable;

    [CreateAssetMenu(fileName = "Coins", menuName = "Data/Currency/Coins", order = 0)]
    public class Coins : ScriptableObject
    {
        public IntVariable Balance;
        public IntVariable ProposedBalanceChange;
    } 
}
