using System;

namespace LooCast.Variable
{
    [Serializable]
    public class FloatVariable : Variable<float>
    {
        public FloatVariable(float value) : base(value)
        {

        }
    }
}