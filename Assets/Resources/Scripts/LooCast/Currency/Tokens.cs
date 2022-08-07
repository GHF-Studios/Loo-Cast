using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Currency
{
    using Variable;

    [CreateAssetMenu(fileName = "Tokens", menuName = "Data/Currency/Tokens", order = 0)]
    public class Tokens : ScriptableObject
    {
        public IntVariable Balance;
        public IntVariable ProposedBalanceChange;
    } 
}
