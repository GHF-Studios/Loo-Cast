using System;

namespace LooCast.Variable
{
    [Serializable]
    public class BoolVariable : Variable<bool>
    {
        public BoolVariable(bool value) : base(value)
        {

        }
    }
}