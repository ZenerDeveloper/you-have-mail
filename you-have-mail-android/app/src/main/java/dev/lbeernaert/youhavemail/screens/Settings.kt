package dev.lbeernaert.youhavemail.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import dev.lbeernaert.youhavemail.R
import dev.lbeernaert.youhavemail.components.AsyncScreen
import dev.lbeernaert.youhavemail.service.ServiceWrapper


@OptIn(ExperimentalMaterialApi::class)
@Composable
fun Settings(
    service: ServiceWrapper,
    onBackClicked: () -> Unit,
    onPollIntervalUpdate: (ULong) -> Unit
) {
    val secondsStr = stringResource(id = R.string.seconds)
    val minutesStr = stringResource(id = R.string.minutes)
    AsyncScreen(
        title = stringResource(id = R.string.settings),
        onBackClicked = onBackClicked
    ) { _, runTask ->

        val pollIntervalValue by service.getPollIntervalValueStateFlow().collectAsState()
        val updatingPollIntervalLabel = stringResource(id = R.string.update_poll_interval)

        Column(
            modifier = Modifier
                .padding(20.dp)
                .fillMaxSize()
        ) {
            var expanded by remember { mutableStateOf(false) }
            val items = listOf(15UL, 30UL, 60UL, 150UL, 300UL, 600UL, 1800UL, 3600UL)
            var selectedIndex by remember { mutableStateOf(0) }

            val onPollIntervalModified: () -> Unit = {
                runTask(updatingPollIntervalLabel) {
                    onPollIntervalUpdate(items[selectedIndex])
                }
            }

            Text(
                text = stringResource(id = R.string.poll_interval),
                style = MaterialTheme.typography.caption
            )
            Spacer(modifier = Modifier.padding(10.dp))
            Text(
                text = stringResource(id = R.string.poll_interval_desc),
                style = MaterialTheme.typography.subtitle1
            )
            Spacer(modifier = Modifier.padding(10.dp))
            TextField(
                modifier = Modifier
                    .fillMaxWidth(),
                value = secondsToText(
                    pollIntervalValue,
                    secondsStr,
                    minutesStr
                ),
                readOnly = true,
                singleLine = true,
                onValueChange = {},
                trailingIcon = {
                    ExposedDropdownMenuDefaults.TrailingIcon(
                        expanded = expanded,
                        onIconClick = { expanded = true })
                },
                colors = ExposedDropdownMenuDefaults.textFieldColors(),
            )
            Spacer(modifier = Modifier.padding(10.dp))
            Box(
                modifier = Modifier
                    .padding(20.dp)
                    .fillMaxSize()
                    .wrapContentSize(Alignment.TopStart)
            ) {
                DropdownMenu(
                    expanded = expanded,
                    onDismissRequest = { expanded = false },
                    modifier = Modifier
                        .fillMaxWidth()
                ) {
                    items.forEachIndexed { index, s ->
                        DropdownMenuItem(onClick = {
                            expanded = false
                            selectedIndex = index
                            onPollIntervalModified()
                        }) {
                            Text(text = secondsToText(s, secondsStr, minutesStr))
                        }
                    }
                }
            }
        }
    }
}

fun secondsToText(seconds: ULong, sec: String, min: String): String {
    return if (seconds < 60UL) {
        "$seconds $sec"
    } else {
        "${(seconds / 60UL)} $min"
    }
}