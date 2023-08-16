using LooCast;
using UnityEngine;
using UnityEngine.UI;

public class DeveloperConsole : MonoBehaviour
{
    #region Fields
    public Text ConsoleOutput;
    public InputField ConsoleInput;
    public Scrollbar Scrollbar;
    #endregion

    #region Unity Callbacks
    private void Awake()
    {
        UpdateLog(LooCastApplication.Log);
        LooCastApplication.OnLogUpdated += UpdateLog;
    }
    #endregion

    #region Methods
    private void UpdateLog(string log)
    {
        ConsoleOutput.text = log;
        Scrollbar.value = 0;
    }
    #endregion
}