using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute
{
    [CreateAssetMenu(fileName = "LuckAttribute", menuName = "Data/Attribute/LuckAttribute", order = 0)]
    public class LuckAttribute : Attribute
    {
        public override string AttributeName
        {
            get
            {
                return "Luck";
            }
        }
    } 
}
