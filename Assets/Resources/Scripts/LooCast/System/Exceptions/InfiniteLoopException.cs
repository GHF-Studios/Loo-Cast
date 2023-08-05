using System;

namespace LooCast.System.Exceptions
{
    public class InfiniteLoopException : Exception
    {
        #region Constructors
        public InfiniteLoopException() : base("An infinite loop has been detected!")
        {
        }
        #endregion
    }
}
