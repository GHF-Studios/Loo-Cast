namespace LooCast.System
{
    public interface ISingletonInstanceProvider
    {
        #region Properties
        IUnityInstance LooCastUnityInstance { get; }
        #endregion
    }
}