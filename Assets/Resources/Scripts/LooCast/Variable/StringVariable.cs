using System;

namespace LooCast.Variable
{
    [Serializable]
    public class StringVariable : Variable<string>
    {
        public StringVariable(string value) : base(value)
        {

        }
    }
}