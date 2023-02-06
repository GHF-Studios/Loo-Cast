using System;

namespace LooCast.Variable
{
    [Serializable]
    public class IntVariable : Variable<int>
    {
        public IntVariable(int value) : base(value)
        {

        }
    }
}