using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute
{
    [CreateAssetMenu(fileName = "WisdomAttribute", menuName = "Data/Attribute/WisdomAttribute", order = 0)]
    public class WisdomAttribute : Attribute
    {
        public override string AttributeName
        {
            get
            {
                return "Wisdom";
            }
        }
    } 
}
