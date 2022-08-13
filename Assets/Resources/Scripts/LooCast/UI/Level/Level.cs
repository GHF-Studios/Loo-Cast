using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Level
{
    public abstract class Level : MonoBehaviour
    {
        public Text Text;

        public abstract void Refresh();
    }
}
